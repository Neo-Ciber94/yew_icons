use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_scissors (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < circle cx = "6" cy = "6" r = "3" /> { "
  " } < circle cx = "6" cy = "18" r = "3" /> { "
  " } < line x1 = "20" y1 = "4" x2 = "8.12" y2 = "15.88" /> { "
  " } < line x1 = "14.47" y1 = "14.48" x2 = "20" y2 = "20" /> { "
  " } < line x1 = "8.12" y1 = "8.12" x2 = "12" y2 = "12" /> </ svg > } }