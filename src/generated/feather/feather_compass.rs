use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_compass (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < circle cx = "12" cy = "12" r = "10" /> { "
  " } < polygon points = "16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76" /> </ svg > } }