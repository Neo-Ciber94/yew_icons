use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn font_awesome_solid_voicemail (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 640 512" >< path d = "M495.1 96c-53.13 0-102 29.25-127 76.13c-25 46.88-22.25 103.8 7.25 147.9H263.7c36.63-54.88 31.25-127.8-13-176.8c-44.38-48.87-116.4-61.37-174.6-30.25s-87.88 97.88-71.75 162c16 64 73.63 108.1 139.6 108.1h352C575.5 384 640 319.5 640 240S575.5 96 495.1 96zM63.99 240c0-44.12 35.88-80 80-80s80 35.88 80 80s-35.88 79.1-80 79.1S63.99 284.1 63.99 240zM495.1 320c-44.13 0-80-35.88-80-79.1s35.88-80 80-80s80 35.88 80 80S540.1 320 495.1 320z" /></ svg > } }