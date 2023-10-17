use screeps::RoomName;

pub struct RoomManager{

}

impl RoomManager {
    pub fn setup(){
        ROOM_MANAGERS.with(|room_managers| {
            let mut room_managers = room_managers.borrow_mut();


        })

    }
}

thread_local! {
  pub static ROOM_MANAGERS: RefCell<HashMap<RoomName,RoomManager>> = RefCell::new(HashMap::new());
}
