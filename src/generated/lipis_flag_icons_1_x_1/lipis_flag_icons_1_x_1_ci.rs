use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_ci (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-ci" viewBox = "0 0 512 512" > < g fill - rule = "evenodd" > { "
    " } < path fill = "#00cd00" d = "M341.5 0H512v512H341.5z" /> { "
    " } < path fill = "#ff9a00" d = "M0 0h170.3v512H0z" /> { "
    " } < path fill = "#fff" d = "M170.3 0h171.2v512H170.3z" /> { "
  " } </ g > </ svg > } }