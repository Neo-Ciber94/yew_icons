use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn bootstrap_toggle_on (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/twbs/icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" class = "bi bi-toggle-on" viewBox = "0 0 16 16" > < path d = "M5 3a5 5 0 0 0 0 10h6a5 5 0 0 0 0-10H5zm6 9a4 4 0 1 1 0-8 4 4 0 0 1 0 8z" /> </ svg > } }