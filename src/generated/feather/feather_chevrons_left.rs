use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_chevrons_left (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < polyline points = "11 17 6 12 11 7" /> { "
  " } < polyline points = "18 17 13 12 18 7" /> </ svg > } }