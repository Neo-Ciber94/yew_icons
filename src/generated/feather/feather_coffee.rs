use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn feather_coffee (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/feathericons/feather - Licensed under MIT" { width } { height } { onclick } fill = "none" stroke = "currentColor" stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" > < path d = "M18 8h1a4 4 0 0 1 0 8h-1" /> { "
  " } < path d = "M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z" /> { "
  " } < line x1 = "6" y1 = "1" x2 = "6" y2 = "4" /> { "
  " } < line x1 = "10" y1 = "1" x2 = "10" y2 = "4" /> { "
  " } < line x1 = "14" y1 = "1" x2 = "14" y2 = "4" /> </ svg > } }