use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_minimize_2 (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < polyline points = "4 14 10 14 10 20" /> { "
  " } < polyline points = "20 10 14 10 14 4" /> { "
  " } < line x1 = "14" y1 = "10" x2 = "21" y2 = "3" /> { "
  " } < line x1 = "3" y1 = "21" x2 = "10" y2 = "14" /> </ svg > } }