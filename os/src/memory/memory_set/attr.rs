use crate::memory::paging::PageEntry;

#[derive(Clone,Debug)]
pub struct MemoryAttr {
    user : bool,    // 用户态是否可访问
    readonly : bool,    // 是否只读
    execute : bool,      // 是否可执行
}

impl MemoryAttr {
    pub fn new() -> Self{
        MemoryAttr {
            user : false,
            readonly : false,
            execute : false,
        }
    }

    pub fn set_user(mut self) -> Self {
        self.user = true;
        self
    }

    pub fn set_readonly(mut self) -> Self {
        self.readonly = true;
        self
    }

    pub fn set_execute(mut self) -> Self {
        self.execute = true;
        self
    }

    pub fn apply(&self, entry : &mut PageEntry) {
        entry.set_present(true);    // 设置页表项存在
        entry.set_user(self.user);  // 设置用户态访问权限
        entry.set_writable(!self.readonly); //设置写权限
        entry.set_execute(self.execute); //设置可执行权限
    }
}
