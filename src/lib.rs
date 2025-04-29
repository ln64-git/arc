pub mod types;
pub mod utility {
    pub mod encrypt;
}
pub mod command {
    pub mod add;
    pub mod commit;
    pub mod init;
    pub mod list;
    pub mod lock;
    pub mod log;
    pub mod pull;
    pub mod restore;
    pub mod unlock;
}

#[cfg(test)]
mod tests {
    mod cli;
    mod encrypt;
    mod types;
}
