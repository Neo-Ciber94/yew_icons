use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_gm (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-gm" viewBox = "0 0 640 480" > < defs > { "
    " } < clipPath id = "gm-a" > { "
      " } < path fill - opacity = ".7" d = "M0-48h640v480H0z" /> { "
    " } </ clipPath > { "
  " } </ defs > { "
  " } < g fill - rule = "evenodd" stroke - width = "1pt" transform = "translate(0 48)" > { "
    " } < path fill = "red" d = "M0-128h640V85.3H0z" /> { "
    " } < path fill = "#fff" d = "M0 85.3h640V121H0z" /> { "
    " } < path fill = "#009" d = "M0 120.9h640V263H0z" /> { "
    " } < path fill = "#fff" d = "M0 263.1h640v35.6H0z" /> { "
    " } < path fill = "#090" d = "M0 298.7h640V512H0z" /> { "
  " } </ g > </ svg > } }