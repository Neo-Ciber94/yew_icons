use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_jp (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-jp" viewBox = "0 0 640 480" > < defs > { "
    " } < clipPath id = "jp-a" > { "
      " } < path fill - opacity = ".7" d = "M-88 32h640v480H-88z" /> { "
    " } </ clipPath > { "
  " } </ defs > { "
  " } < g fill - rule = "evenodd" stroke - width = "1pt" transform = "translate(88 -32)" > { "
    " } < path fill = "#fff" d = "M-128 32h720v480h-720z" /> { "
    " } < circle cx = "523.1" cy = "344.1" r = "194.9" fill = "#d30000" transform = "translate(-168.4 8.6) scale(.76554)" /> { "
  " } </ g > </ svg > } }