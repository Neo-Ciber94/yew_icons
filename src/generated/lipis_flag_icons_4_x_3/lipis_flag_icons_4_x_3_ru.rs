use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_ru (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-ru" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" stroke - width = "1pt" > { "
    " } < path fill = "#fff" d = "M0 0h640v480H0z" /> { "
    " } < path fill = "#0039a6" d = "M0 160h640v320H0z" /> { "
    " } < path fill = "#d52b1e" d = "M0 320h640v160H0z" /> { "
  " } </ g > </ svg > } }