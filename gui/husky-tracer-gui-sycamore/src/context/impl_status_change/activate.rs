use super::*;

impl DeveloperGuiContext {
    pub(crate) fn activate(&'static self, new_active_trace_id: TraceId) {
        if self.opt_active_trace_id() == Some(new_active_trace_id) {
            // do nothing if already activated
            // without this toggle expansion doesn't work
            return;
        }
        let trace_data = self.trace_data(new_active_trace_id);
        let presentation = self
            .presentation_signal()
            .get()
            .activate_trace_out_of_place(trace_data);
        let needs_figure_canvases =
            self.needs_figure_canvases(Some(new_active_trace_id), &presentation);
        let needs_figure_controls =
            self.needs_figure_controls(Some(new_active_trace_id), &presentation);
        let needs_response = needs_figure_canvases || needs_figure_controls;
        self.ws.send_message(
            HuskyTracerGuiMessageVariant::Activate {
                trace_id: new_active_trace_id,
                needs_figure_canvases,
                needs_figure_controls,
            },
            if needs_response {
                Some(Box::new(move |response| match response.variant {
                    HuskyTracerServerMessageVariant::Activate {
                        new_figure_canvases,
                        new_figure_controls,
                    } => {
                        self.receive_figure_canvases(
                            self.scope,
                            new_figure_canvases
                                .into_iter()
                                .map(|(k, v)| (k, self.alloc_value(v))),
                        );
                        self.receive_figure_controls(self.scope, new_figure_controls.into_iter());
                        self.did_activate(presentation);
                    }
                    HuskyTracerServerMessageVariant::ActivateWithError { .. } => todo!(),
                    _ => panic!("unexpected response {:?}", response),
                }))
            } else {
                {
                    self.did_activate(presentation);
                    None
                }
            },
        );
    }

    fn did_activate(&'static self, presentation: Presentation) {
        self.presentation_signal.set(presentation)
    }
}
