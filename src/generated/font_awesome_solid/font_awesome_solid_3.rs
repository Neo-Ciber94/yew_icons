use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn font_awesome_solid_3 (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "Font Awesome Free 6.1.1 by @fontawesome - https://fontawesome.com License - https://fontawesome.com/license/free (Icons: CC BY 4.0, Fonts: SIL OFL 1.1, Code: MIT License) Copyright 2022 Fonticons, Inc." { width } { height } { onclick } fill = "currentColor" viewBox = "0 0 320 512" >< path d = "M320 344c0 74.98-61.02 136-136 136H103.6c-46.34 0-87.31-29.53-101.1-73.48c-5.594-16.77 3.484-34.88 20.25-40.47c16.75-5.609 34.89 3.484 40.47 20.25c5.922 17.77 22.48 29.7 41.23 29.7H184c39.7 0 72-32.3 72-72s-32.3-72-72-72H80c-13.2 0-25.05-8.094-29.83-20.41C45.39 239.3 48.66 225.3 58.38 216.4l131.4-120.4H32c-17.67 0-32-14.33-32-32s14.33-32 32-32h240c13.2 0 25.05 8.094 29.83 20.41c4.781 12.3 1.516 26.27-8.203 35.19l-131.4 120.4H184C258.1 208 320 269 320 344z" /></ svg > } }