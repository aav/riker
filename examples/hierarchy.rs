extern crate riker;
use riker::actors::*;

use std::time::Duration;

use async_trait::async_trait;

#[derive(Default)]
struct Child;

#[async_trait]
impl Actor for Child {
    type Msg = String;

    async fn recv(&mut self, _ctx: &Context<Self::Msg>, msg: Self::Msg, _sender: Sender) {
        println!("child got a message {}", msg);
    }
}

#[derive(Default)]
struct MyActor {
    child: Option<ActorRef<String>>,
}

// implement the Actor trait
#[async_trait]
impl Actor for MyActor {
    type Msg = String;

    fn pre_start(&mut self, ctx: &Context<Self::Msg>) {
        self.child = Some(ctx.actor_of::<Child>("my-child").unwrap());
    }

    async fn recv(&mut self, _ctx: &Context<Self::Msg>, msg: Self::Msg, sender: Sender) {
        println!("parent got a message {}", msg);
        self.child.as_ref().unwrap().tell(msg, sender);
    }
}

// start the system and create an actor
fn main() {
    let sys = ActorSystem::new().unwrap();

    let my_actor = sys.actor_of::<MyActor>("my-actor").unwrap();

    my_actor.tell("Hello my actor!".to_string(), None);

    println!("Child not added yet");
    sys.print_tree();

    println!("Child added already");
    std::thread::sleep(Duration::from_millis(500));
    sys.print_tree();
}
