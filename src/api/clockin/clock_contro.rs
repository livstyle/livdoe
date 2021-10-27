use poem_openapi::{CombinedAPI, OpenApi, payload::{Json}};

use crate::api::clockin::{ApiTags, ClockInfoRequest, ClockInfoResponse, ClockInfo};

pub struct ClockApi;

#[OpenApi]
impl ClockApi {
    #[oai(path = "/getClockInfo", method = "post", tag = "ApiTags::ClockInfoRequest")]
    async fn clock_info(
        &self,
        att_req: Json<ClockInfoRequest>,
    ) -> ClockInfoResponse {
        println!("访问路径v =================  /api/getClockInfo");
        println!("{:#?}", att_req);
        // PlainText("hello!".to_string())
        let clock_info = ClockInfo {
            group_name: String::from("技术中心"),
            group_id: String::from("technoledage"),
            frequency_id: String::from("asdadasas")
        };
        ClockInfoResponse::Ok(Json(clock_info))
    }


}

pub struct ClockApi1;

#[OpenApi]
impl ClockApi1 {
    #[oai(path = "/getClockInfo1", method = "post", tag = "ApiTags::ClockInfoRequest")]
    async fn clock_info(
        &self,
        att_req: Json<ClockInfoRequest>,
    ) -> ClockInfoResponse {
        println!("访问路径v =================  /api/getClockInfo1");
        println!("{:#?}", att_req);
        // PlainText("hello!".to_string())
        let clock_info = ClockInfo {
            group_name: String::from("技术中心"),
            group_id: String::from("technoledage"),
            frequency_id: String::from("asdadasas")
        };
        ClockInfoResponse::Ok(Json(clock_info))
    }

}

pub fn get_clock_api() -> CombinedAPI<ClockApi, ClockApi1> {
    ClockApi.combine(ClockApi1)
}
