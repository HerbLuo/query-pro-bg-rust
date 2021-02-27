use rocket::http::Status;

pub struct HttpErrorCode {
    pub code: u16,
    pub status: Status,
    pub reason: &'static str
}

macro_rules! ctrs {
    ($($status:expr, $code:expr, $name:ident, $reason:expr),+) => {
        $(
            #[allow(non_upper_case_globals, dead_code)]
            pub const $name: HttpErrorCode = HttpErrorCode { code: $code, status: $status, reason: $reason };
        )+
    }
}

// len     1   2     1    2     u16
// code 0o[01]{大类}{类别}{no}
// 0 请求问题 1 服务端问题
// 大类 00 通用 01 登录相关 02 单词相关 03 用户相关 04 版本相关
// 类别 0 多出 1 不正确  2 缺少 3 过期 4 重复
// 序号 两位

ctrs! {
    // 通用
    Status::BadRequest, 0o0_00_0_00, GeneralRequestError, "请求错误",
    Status::BadRequest, 0o0_00_2_01, MissingParam, "参数缺少",
    Status::BadRequest, 0o0_00_1_01, WrongParam, "参数错误",
    Status::NotFound, 0o0_00_2_01, NotFound, "NotFound",

    // 登录相关
    Status::NotFound, 0o0_01_1_01, LoginFailNotFound, "认证失败",
    Status::NotFound, 0o0_01_1_02, TempTokenNotFound, "认证失败",
    Status::Forbidden, 0o0_01_3_01, TokenExpired, "Token过期",
    Status::Unauthorized, 0o0_01_2_01, Unauthorized, "需要授权",
    Status::Forbidden, 0o0_01_2_02, Forbidden, "禁止访问",
    Status::InternalServerError, 0o1_01_1_01, FailToRegister, "创建账户失败",
    Status::InternalServerError, 0o1_01_1_01, GenTokenFailed, "创建Token失败",

    // 单词相关
    Status::BadRequest, 0o0_02_4_01, UserWordExist, "该单词可能已存在于用户的生词本中",

    Status::BadRequest, 0o0_04_3_01, VersionOutDate, "该版本可能已过期，可尝试全量下载",

    // 服务端问题
    Status::NotImplemented, 0o1_00_2_00, NotImplemented, "还未实现",
    Status::InternalServerError, 0o1_00_0_00, ServerError,  "未知错误"
}
