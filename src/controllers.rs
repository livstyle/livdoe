use poem_openapi::{payload::{PlainText, Json}, OpenApi, ApiResponse, Object};

use crate::service::attendance::{get_name, new_user};
pub struct UserApi;

#[OpenApi]
impl UserApi {
    #[oai(path = "/user", method = "get")]
    async fn user_info(
        &self,
        #[oai(name = "name", in = "query")] name: Option<String>,
    ) -> PlainText<String> {
        println!("访问路径v =================  /api/user");
        match name {
            Some(name) => {

                if let Ok(u) = new_user(&name).await {
                    println!("{:#?}", u);
                }

                if let Ok(user) = get_name().await {
                    println!("{:#?}", user);
                } else {
                    println!("error");
                }
                PlainText(format!("hello, {}!", name))
            },
            None => PlainText("hello!".to_string()),
        }
    }



    #[oai(path="/user_list", method = "get")]
    async fn user_list(&self) -> UserListRes {
        if let Ok(users) = get_name().await {
            UserListRes::Ok(Json(users))
        } else {
            UserListRes::NotFound
        }
    }

}

#[derive(Debug, Object, Clone, Eq, PartialEq)]
pub struct UserCode{pub user_code: Option<String>}

#[derive(ApiResponse)]
pub enum UserListRes {
    #[oai(status = 200)]
    Ok(Json<Vec<UserCode>>),
    #[oai(status = 404)]
    NotFound,
}
