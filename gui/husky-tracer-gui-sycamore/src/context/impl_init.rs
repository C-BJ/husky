use futures::channel::mpsc::Receiver;
use web_sys::KeyboardEvent;

use super::impl_status_change::StatusChange;

use super::*;

impl DeveloperGuiContext {
    pub(super) fn init<'a>(&'static self, read: Receiver<HuskyTracerServerMessage>) {
        self.add_event_listeners_to_dialogues();
        self.send_init_request();
        self.spawn_listening(read)
    }

    fn add_event_listeners_to_dialogues(&'static self) {
        let dialog = restriction_dialog();
        add_event_listener!(dialog, "keydown", move |event: web_sys::UiEvent| {
            let event: KeyboardEvent = event.unchecked_into();
            match event.key().as_str() {
                "Enter" => {
                    let sample_id_value = sample_id_input().value();
                    match sample_id_value.parse::<usize>() {
                        Ok(raw) => {
                            restriction_dialog().close();
                            self.handle_status_change(StatusChange::update_restriction(
                                self,
                                |res| res.set_sample_id(SampleId(raw)),
                            ))
                        }
                        Err(_) => alert!("`{}` is not a valid sample id", sample_id_value),
                    }
                }
                _ => (),
            }
        });
    }

    fn send_init_request(&'static self) {
        let mut gui_message_sender = self.ws.gui_message_sender.clone();
        let request_id = self.ws.issue_request_id();
        self.ws.send_message(
            HuskyTracerGuiMessageVariant::HotReloadRequest,
            Some(Box::new(move |response| match response.variant {
                HuskyTracerServerMessageVariant::HotReload { init_data } => {
                    self.receive_init_data(init_data)
                }
                _ => panic!(),
            })),
        );
    }

    fn receive_init_data<'a>(&'static self, init_data: InitData) {
        // order matters
        *self.trace_nodes.borrow_mut(file!(), line!()) = init_data
            .trace_init_data
            .trace_nodes
            .into_iter()
            .map(|trace_node| TraceNodeState::from_data(self.scope, trace_node))
            .collect();
        *self.subtrace_ids_map.borrow_mut(file!(), line!()) = init_data
            .trace_init_data
            .subtrace_ids_map
            .into_iter()
            .map(|(k, v)| (k, self.alloc_value(v) as &'static [TraceId]))
            .collect();
        *self.trace_stalks.borrow_mut(file!(), line!()) = init_data
            .trace_init_data
            .trace_stalks
            .into_iter()
            .map(|(k, v)| (k, self.alloc_value(v)))
            .collect();
        *self.trace_statss.borrow_mut(file!(), line!()) = init_data
            .trace_init_data
            .trace_statss
            .into_iter()
            .map(|(k, v)| (k, v.map(|v| self.alloc_value(v))))
            .collect();
        *self.figure_canvases.borrow_mut(file!(), line!()) = self
            .alloc_key_value_pairs(init_data.figure_canvases)
            .collect();
        *self.figure_controls.borrow_mut(file!(), line!()) = self
            .alloc_key_signal_pairs(init_data.figure_controls)
            .collect();
        // global control
        self.init_presentation(init_data.presentation.clone());
        // root traces
        self.root_trace_ids_signal
            .set(init_data.trace_init_data.root_trace_ids);
        self.update_trace_listing(init_data.presentation.opt_sample_id());
    }

    fn spawn_listening(&'static self, mut read: Receiver<HuskyTracerServerMessage>) {
        spawn_local({
            async move {
                while let Some(notif) = read.next().await {
                    self.handle_server_notification(notif)
                }
                log::debug!("WebSocket Closed");
            }
        });
    }

    pub(super) fn handle_server_message_str(&'static self, server_message_str: &str) {
        self.handle_server_notification(serde_json::from_str(server_message_str).unwrap())
    }

    fn handle_server_notification(&'static self, server_message: HuskyTracerServerMessage) {
        assert!(server_message.opt_request_id.is_none());
        match server_message.variant {
            _ => panic!(),
        }
    }
}
