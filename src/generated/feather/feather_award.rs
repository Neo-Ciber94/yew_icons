use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_award (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < circle cx = "12" cy = "8" r = "7" /> { "
  " } < polyline points = "8.21 13.89 7 23 12 20 17 23 15.79 13.88" /> </ svg > } }