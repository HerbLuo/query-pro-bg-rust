pub struct Permissions {
    pub table: String, //表名
    pub column: Option<String>, //列级权限
    pub uid_read: Option<String>, //表或虚拟表中的uid字段，用于控制表或列的读取权限
    pub uid_write: Option<String>, // 表或虚拟表中的uid字段，用于控制表或列的读取权限
    pub joiners: Option<String>, // 使用joiners构建虚拟表
}
