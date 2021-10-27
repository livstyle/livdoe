use poem_openapi::{payload::Json, Tags, ApiResponse, Object};


#[derive(Tags)]
pub enum ApiTags {
    /// 获取打卡信息接口的入参格式
    ClockInfoRequest,
}

/// 创建打卡入参模式
#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct ClockInfoRequest {

    /// 用户编号
    #[oai(max_length = 64)]
    pub user_code: Option<String>,

    /// 考勤时间
    #[oai(max_length = 64)]
    pub attendance_time: Option<String>,  

}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct ClockInfo {
    /// 考勤组ID
    #[oai(max_length = 64)]
    pub group_id: String,

    /// 考勤组名称
    #[oai(max_length = 64)]
    pub group_name: String,

    /// 当前班次
    #[oai(max_length = 64)]
    pub frequency_id: String,

}


#[derive(ApiResponse)]
pub enum ClockInfoResponse {

    #[oai(status = 200)]
    Ok(Json<ClockInfo>),

    #[oai(status = 400)]
    NotFound
}