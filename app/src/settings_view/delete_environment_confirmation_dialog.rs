use warpui::{
    elements::{ChildView, Container, Dismiss, Empty},
    ui_components::components::UiComponent,
    AppContext, Element, Entity, SingletonEntity, TypedActionView, View, ViewContext, ViewHandle,
};

use crate::{
    appearance::Appearance,
    localization::{
        localized_delete_environment_confirmation_description, localized_for_app, UiStringKey,
    },
    server::ids::SyncId,
    settings::AppLocalizationSettings,
    ui_components::dialog::{dialog_styles, Dialog},
    view_components::action_button::{ActionButton, DangerPrimaryTheme, NakedTheme},
};

const DIALOG_WIDTH: f32 = 450.;

pub enum DeleteEnvironmentConfirmationDialogEvent {
    Cancel,
    Confirm(SyncId),
}

#[derive(Debug)]
pub enum DeleteEnvironmentConfirmationDialogAction {
    Cancel,
    Confirm,
}

pub struct DeleteEnvironmentConfirmationDialog {
    pub(crate) visible: bool,
    pub(crate) env_id: Option<SyncId>,
    pub(crate) env_name: String,
    cancel_button: ViewHandle<ActionButton>,
    confirm_button: ViewHandle<ActionButton>,
}

impl DeleteEnvironmentConfirmationDialog {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        let cancel_button = ctx.add_typed_action_view(|_| {
            ActionButton::new("", NakedTheme).on_click(|ctx| {
                ctx.dispatch_typed_action(DeleteEnvironmentConfirmationDialogAction::Cancel);
            })
        });

        let confirm_button = ctx.add_typed_action_view(|_| {
            ActionButton::new("", DangerPrimaryTheme).on_click(|ctx| {
                ctx.dispatch_typed_action(DeleteEnvironmentConfirmationDialogAction::Confirm);
            })
        });

        ctx.subscribe_to_model(&AppLocalizationSettings::handle(ctx), |me, _, _, ctx| {
            me.update_button_labels(ctx);
            ctx.notify();
        });

        let mut dialog = Self {
            visible: false,
            env_id: None,
            env_name: String::new(),
            cancel_button,
            confirm_button,
        };
        dialog.update_button_labels(ctx);
        dialog
    }

    pub fn show(&mut self, env_id: SyncId, env_name: String, ctx: &mut ViewContext<Self>) {
        self.env_id = Some(env_id);
        self.env_name = env_name;
        self.visible = true;
        ctx.notify();
    }

    pub fn hide(&mut self, ctx: &mut ViewContext<Self>) {
        self.visible = false;
        ctx.notify();
    }

    fn update_button_labels(&mut self, ctx: &mut ViewContext<Self>) {
        self.cancel_button.update(ctx, |button, ctx| {
            button.set_label(
                localized_for_app(UiStringKey::SettingsDialogCancel, ctx),
                ctx,
            );
        });
        self.confirm_button.update(ctx, |button, ctx| {
            button.set_label(
                localized_for_app(UiStringKey::SettingsDeleteEnvironmentConfirmButton, ctx),
                ctx,
            );
        });
    }
}

impl Entity for DeleteEnvironmentConfirmationDialog {
    type Event = DeleteEnvironmentConfirmationDialogEvent;
}

impl View for DeleteEnvironmentConfirmationDialog {
    fn ui_name() -> &'static str {
        "DeleteEnvironmentConfirmationDialog"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        if !self.visible {
            return Empty::new().finish();
        }

        let appearance = Appearance::as_ref(app);

        let description =
            localized_delete_environment_confirmation_description(&self.env_name, app);

        let dialog = Dialog::new(
            localized_for_app(UiStringKey::SettingsDeleteEnvironmentTitle, app).to_string(),
            Some(description),
            dialog_styles(appearance),
        )
        .with_bottom_row_child(ChildView::new(&self.cancel_button).finish())
        .with_bottom_row_child(
            Container::new(ChildView::new(&self.confirm_button).finish())
                .with_margin_left(12.)
                .finish(),
        )
        .with_width(DIALOG_WIDTH)
        .build()
        .finish();

        Dismiss::new(dialog)
            .prevent_interaction_with_other_elements()
            .on_dismiss(|ctx, _app| {
                ctx.dispatch_typed_action(DeleteEnvironmentConfirmationDialogAction::Cancel)
            })
            .finish()
    }
}

impl TypedActionView for DeleteEnvironmentConfirmationDialog {
    type Action = DeleteEnvironmentConfirmationDialogAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            DeleteEnvironmentConfirmationDialogAction::Cancel => {
                ctx.emit(DeleteEnvironmentConfirmationDialogEvent::Cancel)
            }
            DeleteEnvironmentConfirmationDialogAction::Confirm => {
                if let Some(env_id) = self.env_id {
                    ctx.emit(DeleteEnvironmentConfirmationDialogEvent::Confirm(env_id));
                }
            }
        }
    }
}
