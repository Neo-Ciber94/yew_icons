use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn font_awesome_solid_feather (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 512 512" >< path d = "M483.4 244.2L351.9 287.1h97.74c-9.874 10.62 3.75-3.125-46.24 46.87l-147.6 49.12h98.24c-74.99 73.12-194.6 70.62-246.8 54.1l-66.14 65.99c-9.374 9.374-24.6 9.374-33.98 0s-9.374-24.6 0-33.98l259.5-259.2c6.249-6.25 6.249-16.37 0-22.62c-6.249-6.249-16.37-6.249-22.62 0l-178.4 178.2C58.78 306.1 68.61 216.7 129.1 156.3l85.74-85.68c90.62-90.62 189.8-88.27 252.3-25.78C517.8 95.34 528.9 169.7 483.4 244.2z" /></ svg > } }