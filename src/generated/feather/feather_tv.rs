use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_tv (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < rect x = "2" y = "7" width = "20" height = "15" rx = "2" ry = "2" /> { "
  " } < polyline points = "17 2 12 7 7 2" /> </ svg > } }