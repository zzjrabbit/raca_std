use core::fmt;

use alloc::{string::String, vec::Vec};

use crate::println;

/// The oepn mode \
/// read means read.
/// write means read & write.
#[repr(C)]
pub enum OpenMode {
    Read = 0,
    Write = 1,
}

/// The file descriptor structure
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FileDescriptor(pub usize, pub(crate) bool);

impl FileDescriptor {
    /// This opens a file at `path` with mode `open_mode` \
    /// The file will be closed when the file descriptor is dropped.
    pub fn open(path: &str, open_mode: OpenMode) -> Result<Self, ()> {
        const OPEN_SYSCALL_ID: u64 = 2;
        let fd = crate::syscall(
            OPEN_SYSCALL_ID,
            path.as_ptr() as usize,
            path.len(),
            open_mode as usize,
            0,
            0,
        );
        if fd == 0 {
            Err(())
        } else {
            Ok(Self(fd, false))
        }
    }

    /// this opens a pipe, the first FileDescriptor is the read side, and the next one is the write side.
    /// You can use one of them as stdin or stdout stream for the sub process.
    pub fn open_pipe() -> Result<(Self, Self), ()> {
        const OPEN_SYSCALL_ID: u64 = 12;
        let mut buf = [0usize; 2];
        let code = crate::syscall(OPEN_SYSCALL_ID, buf.as_mut_ptr() as usize, 0, 0, 0, 0);
        println!("{:?}", buf);
        if code == 0 {
            Err(())
        } else {
            Ok((Self(buf[0], false), Self(buf[1], false)))
        }
    }

    /// Read something to the buffer.
    pub fn read(&self, buffer: &mut [u8]) -> usize {
        assert_ne!(self.1, true, "This File Descriptor had been closed!");

        const READ_SYSCALL_ID: u64 = 4;
        crate::syscall(
            READ_SYSCALL_ID,
            self.0,
            buffer.as_ptr() as usize,
            buffer.len(),
            0,
            0,
        )
    }

    /// This function will read until the buffer is full.
    pub fn read_exact(&self, buffer: &mut [u8]) {
        let mut readed = 0;
        while readed < buffer.len() {
            let read_size = self.read(&mut buffer[readed..]);
            readed += read_size;
        }
    }

    /// Write something to the file.
    pub fn write(&self, buffer: &[u8]) -> usize {
        assert_ne!(self.1, true, "This File Descriptor had been closed!");

        const WRITE_SYSCALL_ID: u64 = 3;
        crate::syscall(
            WRITE_SYSCALL_ID,
            self.0,
            buffer.as_ptr() as usize,
            buffer.len(),
            0,
            0,
        )
    }

    /// Seek to the specified position.
    pub fn seek(&self, offset: usize) -> usize {
        assert_ne!(self.1, true, "This File Descriptor had been closed!");

        const LSEEK_SYSCALL_ID: u64 = 10;
        crate::syscall(LSEEK_SYSCALL_ID, self.0, offset, 0, 0, 0)
    }

    /// Get the size of the file.
    pub fn size(&self) -> usize {
        assert_ne!(self.1, true, "This File Descriptor had been closed!");

        const FSIZE_SYSCALL_ID: u64 = 11;
        crate::syscall(FSIZE_SYSCALL_ID, self.0, 0, 0, 0, 0)
    }

    pub(self) fn close(&mut self) {
        self.1 = true;

        const CLOSE_SYSCALL_ID: u64 = 9;
        crate::syscall(CLOSE_SYSCALL_ID, self.0, 0, 0, 0, 0);
    }

    /// Get the type of the descriptor
    pub fn get_type(&self) -> FileType {
        const GET_TYPE_SYSCALL_ID: u64 = 19;
        let ty = crate::syscall(GET_TYPE_SYSCALL_ID, self.0, 0, 0, 0, 0) as usize;
        match ty {
            0 => FileType::Dir,
            1 => FileType::File,
            _ => unreachable!(),
        }
    }
}

impl fmt::Write for FileDescriptor {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        if self.write(s.as_bytes()) != s.as_bytes().len() {
            fmt::Result::Err(fmt::Error::default())
        } else {
            Ok(())
        }
    }
}

impl Drop for FileDescriptor {
    fn drop(&mut self) {
        self.close();
    }
}

/// File types
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug, Default)]
pub enum FileType {
    Dir = 0,
    #[default]
    File = 1,
}

/// File Info
#[repr(C)]
#[derive(Clone, Debug)]
pub struct FileInfo {
    pub name: String,
    pub ty: FileType,
}

impl Default for FileInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            ty: FileType::Dir,
        }
    }
}

impl FileInfo {
    /// Get the list of files and directories in the directory.
    pub fn list(path: String) -> Vec<Self> {
        fn dir_item_num(path: String) -> usize {
            const DIR_ITEM_NUM_SYSCALL: u64 = 14;
            crate::syscall(
                DIR_ITEM_NUM_SYSCALL,
                path.as_ptr() as usize,
                path.len(),
                0,
                0,
                0,
            )
        }

        #[derive(Default, Clone)]
        struct TemporyInfo {
            name: &'static [u8],
            ty: FileType,
        }

        let len = dir_item_num(path.clone());
        let buf = alloc::vec![TemporyInfo::default();len];

        const LIST_DIR_SYSCALL: u64 = 13;
        crate::syscall(
            LIST_DIR_SYSCALL,
            path.as_ptr() as usize,
            path.len(),
            buf.as_ptr() as usize,
            0,
            0,
        );

        let mut infos = Vec::new();
        for info in buf.iter() {
            infos.push(FileInfo {
                name: String::from_utf8(info.name.to_vec()).unwrap(),
                ty: info.ty,
            })
        }
        infos
    }
}

/// Change cwd to the specified directory.
pub fn change_cwd(path: String) {
    const CHANGE_CWD_SYSCALL: u64 = 15;
    crate::syscall(
        CHANGE_CWD_SYSCALL,
        path.as_ptr() as usize,
        path.len(),
        0,
        0,
        0,
    );
}

/// Get the cwd
pub fn get_cwd() -> String {
    const GET_CWD_SYSCALL: u64 = 16;
    let ptr = crate::syscall(GET_CWD_SYSCALL, 0, 0, 0, 0, 0);
    let path_buf_ptr = unsafe { (ptr as *const u64).read() };
    let path_buf_len = unsafe { (ptr as *const usize).add(1).read() };
    let path_buf = unsafe { core::slice::from_raw_parts(path_buf_ptr as *const u8, path_buf_len) };
    String::from_utf8(path_buf.to_vec()).unwrap()
}

/// create a file or directory at the specified path
pub fn create(path: String, ty: FileType) -> Result<FileDescriptor, ()> {
    const CREATE_SYSCALL_ID: u64 = 17;
    let fd = crate::syscall(
        CREATE_SYSCALL_ID,
        path.as_ptr() as usize,
        path.len(),
        ty as usize,
        0,
        0,
    );
    if fd == 0 {
        Err(())
    } else {
        Ok(FileDescriptor(fd, false))
    }
}

/// Mount the specified partition to the specified path.
pub fn mount(path: String, partition: String) -> Result<(), ()> {
    const MOUNT_SYSCALL_ID: u64 = 20;
    crate::syscall(
        MOUNT_SYSCALL_ID,
        path.as_ptr() as usize,
        path.len(),
        partition.as_ptr() as usize,
        partition.len(),
        0,
    );
    Ok(())
}
