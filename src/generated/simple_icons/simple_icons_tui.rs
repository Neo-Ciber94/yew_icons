use crate :: IconProps ; # [inline (never)] pub fn simple_icons_tui (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M24 4.5167a2.117 2.117 0 01-2.117 2.117 2.117 2.117 0 01-2.117-2.117 2.117 2.117 0 012.117-2.117A2.117 2.117 0 0124 4.5168zM1.1397 7.7475h5.7055c.5642 0 .9806.1772 1.1465.9716.185.8836.1129 1.4986-.8858 1.5686l-1.7909.132c1.318 8.3303 9.0277 11.0453 13.2221 2.073.6952-1.485.922-1.7548 1.6826-1.5663 1.0314.2561 1.1724.7899.677 2.2828-3.6234 11.0566-15.8186 12.166-18.211-2.6044l-1.4546.105C.0463 10.7942 0 9.7956 0 9.2404c0-1.0992.4074-1.493 1.1397-1.493z" /></ svg > } }