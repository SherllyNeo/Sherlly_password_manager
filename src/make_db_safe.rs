
pub fn encode(text: String) -> String {

    text.replace("\"","🛍️").replace("\'","🥵").replace("}}","😇").replace("{{","🫥")


}

pub fn decode(text: String) -> String {

    text.replace("🛍️","\"").replace("🥵","\'").replace("😇","}}").replace("🫥","{{")

}
