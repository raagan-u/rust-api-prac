use actix_web::{get, post, put, delete, web::{Data, Json}, web, Responder, HttpResponse};

use crate::{
	models::poll_data::{Poll, PollRequest},
	services::db::Database
};

#[post("/poll")]
pub async fn create_poll (db: Data<Database>, request: Json<PollRequest>)-> HttpResponse {
	match db
		.create_poll(
			Poll::try_from(PollRequest {
				poll_id: request.poll_id.clone(),
				title: request.title.clone(),
				description: request.description.clone(),
				options: request.options.clone(),
				status: request.status.clone(),
				expiration_date: request.expiration_date.clone()
			})
			.expect("Error converting OwnerRequest to Owner."),
		)
		.await
	{
			Ok(poll) => HttpResponse::Ok().json(poll),
			Err(err) => 	HttpResponse::InternalServerError().body(err.to_string()),
	}
}

#[get("/polls")]
pub async fn get_poll(db: Data<Database>)-> HttpResponse{
	match db.get_poll().await {
		Ok(polls) => HttpResponse::Ok().json(polls),
		Err(err) => HttpResponse::InternalServerError().body(err.to_string()),	
	}
}

#[put("/poll")]
pub async fn update_poll(db: Data<Database>, request: Json<PollRequest>)-> HttpResponse {
	match db
		.update_poll(
			Poll::try_from(PollRequest {
				poll_id: request.poll_id.clone(),
				title: request.title.clone(),
				description: request.description.clone(),
				options: request.options.clone(),
				status: request.status.clone(),
				expiration_date: request.expiration_date.clone()
			})
			.expect("Error while converting"),
		)
		.await
		{
			Ok(poll) => HttpResponse::Ok().json(poll),
			Err(err) => 	HttpResponse::InternalServerError().body(err.to_string()),
		}
}

#[delete("/poll/{poll_id}")]
async fn delete_poll(db: web::Data<Database>, path: web::Path<i64>) -> impl Responder {
    let poll_id = path.into_inner(); // Extract the poll_id from the path

    // Perform deletion from the database
    match db.delete_poll(poll_id).await {
        Ok(_) => HttpResponse::Ok().body("Poll deleted successfully"),
        Err(e) => {
            eprintln!("Error deleting poll: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to delete poll")
        }
    }
}