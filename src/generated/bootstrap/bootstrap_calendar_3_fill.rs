use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn bootstrap_calendar_3_fill (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/twbs/icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" class = "bi bi-calendar3-fill" viewBox = "0 0 16 16" > < path d = "M0 2a2 2 0 0 1 2-2h12a2 2 0 0 1 2 2H0zm0 1v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V3H0z" /> </ svg > } }