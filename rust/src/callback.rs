//! Office menu bot.
//! Callback from inline button

use strum::{AsRefStr, EnumString, };

use teloxide::{
   prelude::*,
   types::{CallbackQuery, },
};
use crate::{db, menu, states::*};
// use crate::database as db;
// use crate::navigation;
// use crate::registration;
use crate::loc::*;

#[derive(AsRefStr, EnumString, )]
pub enum Command {
   #[strum(to_string = "pas")]
   Pass(i32), // make the specified node active
   #[strum(to_string = "pan")]
   PassNow(i32), // like Pass but only among opened
   #[strum(to_string = "inc")]
   IncAmount(i32), // add 1pcs of node to cart and return to Pass mode
   #[strum(to_string = "inn")]
   IncAmountNow(i32), // add 1pcs of node to cart and return to PassNow mode
   #[strum(to_string = "dec")]
   DecAmount(i32), // remove 1pcs of node from cart and return to Pass mode
   #[strum(to_string = "den")]
   DecAmountNow(i32), // remove 1pcs of node from cart and return to PassNow mode
   #[strum(to_string = "tic")]
   TicketMake(i32), // start ordering through the bot
   #[strum(to_string = "tca")]
   TicketCancel(i32), // cancel ticket
   #[strum(to_string = "tne")]
   TicketNext(i32), // next stage for ticket
   #[strum(to_string = "tco")]
   TicketConfirm(i32), // finish ticket
   Unknown,
}

impl Command {
   pub fn parse(s: &str) -> Self {
      // Looking for the commands with arguments
      let cmd = s.get(..3).unwrap_or_default();
      let arg = s.get(3..)
      .unwrap_or_default()
      .parse().unwrap_or_default();

      if cmd == Self::Pass(0).as_ref() {
         Command::Pass(arg)
      } else if cmd == Self::PassNow(0).as_ref() {
         Command::PassNow(arg)
      } else if cmd == Self::IncAmount(0).as_ref() {
         Command::IncAmount(arg)
      } else if cmd == Self::IncAmountNow(0).as_ref() {
         Command::IncAmountNow(arg)
      } else if cmd == Self::DecAmount(0).as_ref() {
         Command::DecAmount(arg)
      } else if cmd == Self::DecAmountNow(0).as_ref() {
         Command::DecAmountNow(arg)
      } else if cmd == Self::TicketMake(0).as_ref() {
         Command::TicketMake(arg)
      } else if cmd == Self::TicketCancel(0).as_ref() {
         Command::TicketCancel(arg)
      } else if cmd == Self::TicketNext(0).as_ref() {
         Command::TicketNext(arg)
      } else if cmd == Self::TicketConfirm(0).as_ref() {
         Command::TicketConfirm(arg)
      } else {
         Command::Unknown
      }
   }

   pub fn buttton_caption(&self, _loc_tag: LocaleTag) -> String {
      match self {
         Self::TicketCancel(_) => loc("Cancel"),
         Self::TicketNext(_) => loc("Next"),
         Self::TicketConfirm(_) => loc("Confirm"),
         _ => String::from("callback::button_caption unsupported command"),
      }
   }
}


pub async fn update(bot: Bot, q: CallbackQuery, tag: LocaleTag) -> HandlerResult {
   async fn do_inc(bot: &Bot, q: CallbackQuery, node_id: i32, mode: WorkTime, tag: LocaleTag) -> Result<String, String> {
      // Increment amount in database and reload node
      let user_id = q.from.id.0;
    //   db::orders_amount_inc(user_id, node_id).await?;
      menu::view(bot, q, node_id, mode, tag).await?;
      Ok(loc("Added"))
   }

   async fn do_dec(bot: &Bot, q: CallbackQuery, node_id: i32, mode: WorkTime, tag: LocaleTag) -> Result<String, String> {
      // Decrement amount in database and reload node
      let user_id = q.from.id.0;
      // db::orders_amount_dec(user_id, node_id).await?;
      menu::view(bot, q, node_id, mode, tag).await?;
      Ok(loc("Removed"))
   }

   let query_id = q.id.to_owned();

   // Parse and process commands by receiving a message to send back
   let input = q.data.to_owned().unwrap_or_default();
   let cmd = Command::parse(&input);
   let msg = match cmd {
      Command::Pass(node_id) => {
         menu::view(&bot, q, node_id, WorkTime::All, tag).await?;
         loc("All places")
      }
      Command::PassNow(node_id) => {
         menu::view(&bot, q, node_id, WorkTime::Now, tag).await?;
         loc("Open now")
      }
      Command::IncAmount(node_id) => do_inc(&bot, q, node_id, WorkTime::All, tag).await?,
      Command::IncAmountNow(node_id) => do_inc(&bot, q, node_id, WorkTime::Now, tag).await?,
      Command::DecAmount(node_id) => do_dec(&bot, q, node_id, WorkTime::All, tag).await?,
      Command::DecAmountNow(node_id) => do_dec(&bot, q, node_id, WorkTime::All, tag).await?,
      // Command::TicketMake(node_id) => registration::make_ticket(&bot, q, node_id, tag).await?,
      // Command::TicketCancel(node_id) => registration::cancel_ticket(&bot, q, node_id, tag).await?,
      // Command::TicketNext(node_id) => registration::next_ticket(&bot, node_id, tag).await?,
      // Command::TicketConfirm(node_id) => registration::confirm_ticket(&bot, node_id, tag).await?,
      Command::Unknown => format!("callback.update | Unknowm command {}", input),
      _ => format!("callback.update | Command {} - not implemented", input)
   };

   // Sending a response that is shown in a pop-up window
   bot.answer_callback_query(query_id)
   .text(msg)
   .await
   .map_err(|err| format!("inline::update {}", err))?;

   Ok(())
}