use std::collections::HashMap;
use std::option::Option;
use tarpc::util::Message;
// use self::tokio_core::reactor;

use super::types::Action;

service! {
    rpc get_action(player_id: usize) -> Action | Message;
}

#[derive(Clone)]
pub struct HostServer {
    pub last_actions: HashMap<usize, Action>,
}

impl FutureService for HostServer {
    type GetActionFut = Result<Action, Message>;

    fn get_action(&self, player_id: usize) -> Self::GetActionFut {
        // Hmm, we probably want to block asynchronously until this action is ready.
        Ok(*self.last_actions.get(&player_id).unwrap())
    }
}

#[derive(Clone)]
pub struct PlayerServer {
    pub last_action: Option<Action>,
}

impl FutureService for PlayerServer {
    type GetActionFut = Result<Action, Message>;
    fn get_action(&self, foo: usize) -> Self::GetActionFut {
        Ok(self.last_action.unwrap())
    }
}

// these are just snippets, to be inserted into the relevant places
// this is actually like a constructor for HostServer
/*
fn serve_as_host(address: &String, mut reactor: &reactor::Core) {
    let (handle, server) = HostServer.listen(address.first_socket_addr(),
                               &reactor.handle(),
                               server::Options::default())
                       .unwrap();
    reactor.handle().spawn(server);
}

// as HostServer
fn get_action_host(player_id: usize, mut reactor: &reactor::Core) -> Action{
    // where is handle???
    let options = client::Options::default().handle(reactor.handle());
    reactor.run(FutureClient::connect(handle.addr(), options)
            .map_err(tarpc::Error::from)
            .and_then(|client| client.action_from_player(player_id))
        .unwrap()
}

*/
