use crate::syscall;

pub fn debug(string: &str) {
    syscall(0, string.as_ptr() as usize, string.len(), 0, 0, 0);
}

