#[macro_use]
extern crate rocket;
extern crate uuid;
use bangbang::geometry::Shape3D;
use bangbang::geometry::Vertex3D;
use bangbang::logger_fairing::Logger;
use rocket::response::status::NotFound;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::serde::Serialize;
use rocket::State;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
struct Version(u32);
impl Default for Version {
    fn default() -> Self {
        Version(1)
    }
}

struct AppState {
    objects: Arc<RwLock<HashMap<Uuid, bangbang::geometry::Vertex3D>>>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct CreateRequest {
    version: Version,
    object_id: String,
    location: Vertex3D,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct CreateResponse {
    version: Version,
    object_id: String,
    location: Vertex3D,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct IndexResponse {
    version: Version,
    search: Shape3D,
    object_ids: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct ReadResponse {
    version: Version,
    location: Vertex3D,
    object_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct UpdateRequest {
    version: Version,
    location: Vertex3D,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct UpdateResponse {
    version: Version,
    object_id: String,
    location: Vertex3D,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
struct DeleteResponse {
    version: Version,
    object_id: String,
}

#[post("/", format = "application/json", data = "<request>")]
fn create(state: &State<AppState>, request: Json<CreateRequest>) -> Result<Json<CreateResponse>,NotFound<String>> {
    // parse id
    let id = Uuid::try_parse(request.object_id.as_str())
    .expect("Unable to parse id");
    // parse point

    println!(
        "CREATE {} at {}",
        id.as_simple().to_string(),
        request.location.to_string()
    );

    // start tracking object _uuid at given location
    let arc = state.objects.clone();
    let mut objects = arc.write()
    .expect("Unable to get write lock on state");
    objects.insert(id, request.location);

    Ok(Json::from(CreateResponse {
        version: Version(1),
        object_id: id.as_simple().to_string(),
        location: request.location,
    }))
}

#[get("/<x>/<y>/<z>/<radius>")]
fn index(state: &State<AppState>, x: f32, y: f32, z: f32, radius: f32) -> Result<Json<IndexResponse>,NotFound<String>> {
    let sph = Shape3D::Sphere { center: Vertex3D { x, y, z }, radius };
    let pt: Vertex3D = Vertex3D { x, y, z };
    println!("INDEX center={}, r={}", pt.to_string(), radius);

    let mut object_ids = Vec::new();
    let arc = state.objects.clone();
    let objects = arc.read()
    .expect("Unable to get read lock on state");
    // loop through objects and test within radius of x,y,z/r and add to return
    for k in objects.keys() {
        let obj_point = objects.get(k)
        .expect(format!("Unable to find vertex for {}", k.to_string()).as_str());
        println!("Checking object {} at {}", k, obj_point.to_string());

        // TODO check object bbox or cylinder
        if obj_point.is_on_or_inside(&sph) {
            object_ids.push(k.simple().to_string().to_owned());
        }
    }

    if object_ids.len() <= 0 {
        return Err(NotFound("No matching objects found".to_string()));
    }

    // TODO: long polling/pubsub

    Ok(Json::from(IndexResponse {
        version: Version(1),
        search: sph,
        object_ids: object_ids,
    }))
}

#[get("/<id>")]
fn read(state: &State<AppState>, id: &str) -> Result<Json<ReadResponse>,NotFound<String>> {
    // parse id
    let id = Uuid::try_parse(id)
    .expect("Unable to parse id");

    println!("READ {}", id.as_simple().to_string());

    // TODO: find location of object and return
    let arc = state.objects.clone();
    let objects = arc.read()
    .expect("Unable to get read lock on state");
    let pt = if let Some(pt) = objects.get(&id) {
        pt
    } else {
        return Err(NotFound("Couldn't find object".to_string()));
    };

    // TODO: long polling/pubsub
    Ok(Json::from(ReadResponse {
        version: Version(1),
        location: *pt,
        object_id: id.as_simple().to_string()
    }))
}

#[put("/<id>", format = "application/json", data = "<request>")]
fn update(state: &State<AppState>, id: &str, request: Json<UpdateRequest>) -> Result<Json<UpdateResponse>,NotFound<String>> {
    // parse id
    let id = Uuid::try_parse(id)
    .expect("Unable to parse id");

    println!(
        "UPDATE {} at {}",
        id.as_simple().to_string(),
        request.location.to_string()
    );

    let arc = state.objects.clone();
    let mut objects = arc.write()
    .expect("Unable to get read lock on state");

    if !objects.contains_key(&id) {
        return Err(NotFound("Object was not found".to_string()));
    }


    objects.insert(id, request.location)
    .expect("Object update failed");

    Ok(Json::from(UpdateResponse {
        version: Version(1),
        location: request.location,
        object_id: id.as_simple().to_string()
    }))
}

#[delete("/<id>")]
fn delete(state: &State<AppState>, id: &str) -> Result<Json<DeleteResponse>,NotFound<String>> {
    // parse id
    let id = Uuid::try_parse(id)
    .expect("Unable to parse id");

    println!("DELETE {}", id.as_simple().to_string());

    // stop tracking object _uuid
    let arc = state.objects.clone();
    let mut objects = arc.write()
        .expect("Unable to get read lock on state");
    if objects.remove(&id).is_none() {
        return Err(NotFound("Couldn't find object".to_string()));
    }

    Ok(Json::from(DeleteResponse {
        version: Version(1),
        object_id: id.as_simple().to_string()
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(AppState {
            objects: Arc::new(RwLock::new(HashMap::new())),
        })
        .attach(Logger {})
        .mount("/", routes![create, index, read, update, delete])
}

#[cfg(test)]
mod test {
    use super::*;

    use bangbang::geometry::Shape3D;
    use rocket::http::ContentType;
    use rocket::http::uri::Uri;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    use rocket::serde::json::serde_json;
    use uuid::Uuid;

    const TEST_ID: &str = "f1cc50ec66f14e9e87e2ed0ae8607b9f";

    #[test]
    fn create() {
        let client = Client::tracked(rocket())
        .expect("valid rocket instance");
        let req = CreateRequest {
            version: Version(1),
            object_id: TEST_ID.to_string(),
            location: Vertex3D { x: 0.0, y: 0.0, z: 0.0 },
        };
        let response = client.post(uri!("/"))
            .header(ContentType::JSON)
            .body(serde_json::to_string(&req).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type().unwrap(), ContentType::JSON);
        assert_eq!(response.into_string().unwrap(), serde_json::to_string(&CreateResponse {
            version: Version(1),
            object_id: TEST_ID.to_string(),
            location: Vertex3D { x: 0.0, y: 0.0, z: 0.0 },
        }).unwrap());
    }

    #[test]
    fn index() {
        let r = rocket();

        // set up state
        let state = r.state::<AppState>().unwrap();
        let objects = state.objects.clone();
        objects.write().unwrap().insert(Uuid::try_parse(&TEST_ID.to_string()).unwrap(), Vertex3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });

        let client = Client::tracked(r)
        .expect("valid rocket instance");
        let response = client.get(uri!("/0.0/0.0/0.0/100.0"))
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type().unwrap(), ContentType::JSON);
        assert_eq!(response.into_string().unwrap(), serde_json::to_string(&IndexResponse {
            version: Version(1),
            object_ids: Vec::from([TEST_ID.to_string()]),
            search: Shape3D::Sphere {
                center: Vertex3D { x: 0.0, y: 0.0, z: 0.0 },
                radius: 100.0,
            },
        }).unwrap());
    }

    #[test]
    fn read() {
        let r = rocket();

        // set up state
        let state = r.state::<AppState>().unwrap();
        let objects = state.objects.clone();
        objects.write().unwrap().insert(Uuid::try_parse(&TEST_ID.to_string()).unwrap(), Vertex3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });

        let client = Client::tracked(r)
        .expect("valid rocket instance");
        let path = format!("/{}", TEST_ID.to_string());
        let response = client.get(Uri::parse_any(path.as_str()).unwrap())
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type().unwrap(), ContentType::JSON);
        assert_eq!(response.into_string().unwrap(), serde_json::to_string(&ReadResponse {
            version:Version(1),
            object_id:TEST_ID.to_string(),
            location: Vertex3D { x: 0.0, y: 0.0, z: 0.0 },
        }).unwrap());
    }

    #[test]
    fn read_bad_id() {
        let r = rocket();

        let client = Client::tracked(r)
        .expect("valid rocket instance");
        let path = "/blah";
        let response = client.get(Uri::parse_any(path).unwrap())
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
    }

    #[test]
    fn update() {
        let r = rocket();

        // set up state
        let state = r.state::<AppState>().unwrap();
        let objects = state.objects.clone();
        objects.write().unwrap().insert(Uuid::try_parse(&TEST_ID.to_string()).unwrap(), Vertex3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });

        let req = UpdateRequest {
            version: Version(1),
            location: Vertex3D { x: 1.0, y: 1.0, z: 1.0 },
        };
        let client = Client::tracked(r)
        .expect("valid rocket instance");
        let path = format!("/{}", TEST_ID.to_string());
        let response = client.put(Uri::parse_any(path.as_str()).unwrap())
            .header(ContentType::JSON)
            .body(serde_json::to_string(&req).unwrap())
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type().unwrap(), ContentType::JSON);
        assert_eq!(response.into_string().unwrap(), serde_json::to_string(&UpdateResponse {
            version:Version(1),
            object_id:TEST_ID.to_string(),
            location: Vertex3D { x: 1.0, y: 1.0, z: 1.0 },
        }).unwrap());
    }

    #[test]
    fn delete() {
        let r = rocket();

        // set up state
        let state = r.state::<AppState>().unwrap();
        let objects = state.objects.clone();
        objects.write().unwrap().insert(Uuid::try_parse(&TEST_ID.to_string()).unwrap(), Vertex3D {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        });

        let client = Client::tracked(r)
        .expect("valid rocket instance");
        let path = format!("/{}", TEST_ID.to_string());
        let response = client.delete(Uri::parse_any(path.as_str()).unwrap())
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type().unwrap(), ContentType::JSON);
        assert_eq!(response.into_string().unwrap(), serde_json::to_string(&DeleteResponse {
            version: Version(1),
            object_id: TEST_ID.to_string(),
        }).unwrap());
    }

    #[test]
    fn delete_bad_id() {
        let r = rocket();

        let client = Client::tracked(r)
        .expect("valid rocket instance");
        let path = "/blah";
        let response = client.delete(Uri::parse_any(path).unwrap())
            .header(ContentType::JSON)
            .dispatch();
        assert_eq!(response.status(), Status::InternalServerError);
    }
}
