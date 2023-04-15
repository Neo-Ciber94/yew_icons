use crate :: IconProps ; # [inline (never)] pub fn simple_icons_jitsi (IconProps { icon_id : _ , title , width , height , onclick , oncontextmenu , class , style } : & IconProps) -> yew :: Html { yew :: html ! { < svg role = "img" viewBox = "0 0 24 24" xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/simple-icons/simple-icons - Licensed under CC0; check brand guidelines" width = { width . clone () } height = { height . clone () } onclick = { onclick . clone () } oncontextmenu = { oncontextmenu . clone () } class = { class . clone () } style = { style . clone () } fill = "currentColor" > if let Some (title) = title . clone () { < title > { title } </ title > } < path d = "M18.948 7.828c-.638-.406-1.527-.346-1.8-.317l-.156.003c-.135-.058.004-.397.041-.753.051-.496-.131-1.177-.461-1.722-.159-.262-.216-.287-.098-.413.869-.928 1.005-1.956.729-2.812-.539-1.675-.648-1.939-.631-1.771.068.683-.084 1.637-.188 2.025-.148.55-.654 1.248-1.979 1.884-.311.149-1.565.74-1.788.998-.276.321-.344.644-.474 1.283-.138.676-.189 1.307-.027 2.03.031.141.063.246.081.329a.017.017 0 0 0-.002-.005c.037.147-.023.234-.105.286a1.455 1.455 0 0 1-.252.08l-.003.001c-.203.032-.398.067-.585.104-1.379.237-4.401.958-3.328 4.704.381 1.271 1.124 2.078 1.554 2.226l.015.005c.071.032.148.06.223.075.008.002.012.124-.004.296l-.011.074c-.06.434-.308 1.104-.673 1.113-.146.004-.79-.386-.972-.505-1.02-.67-1.384-1.048-2.108-1.129-.597-.067-1.939 1.093-1.968 3.549-.035 2.999.806 4.49.83 4.534.623-2.824 1.072-3.1 2.689-4.205.128-.087 1.647 1.191 2.012 1.184 1.827-.034 5.102.083 6.689-3.196.031-.063.549.474.595.473.019-.001 2.788-1.234 3.199-6.899.188-2.587-.536-3.206-1.044-3.529zm-2.172-1.846c.08.327.057.667-.041.947-.178.427-.45.651-.79.627a.812.812 0 0 1-.31-.11c-.378-.221-.583-.786-.422-1.225a.183.183 0 0 0 .01-.02c.042-.112.144-.237.275-.364.269-.238.797-.64.854-.64.072.002.341.443.424.785zm-.004-4.911c.006-.058.153.385.201.539.209.665.181.96.134 1.297-.122.858-.579 1.422-.924 1.766-.541.54-.668.618-.432.191.753-1.361.912-2.641 1.021-3.793zm-4.103 4.186c.097-.239.584-.503.989-.739.418-.244 1.747-.59 2.566-1.669.253-.333-.214 1.915-1.417 3.002-.406.367-1.522.557-2.485 1.242-.099.071.022-1.153.347-1.836zm-.258 1.993c.236-.196.664-.477 2.209-.987.191-.063.177.002.27.457.099.486.197.933 1.207 1.178.072.018-.328.772-.443.871-.225.36-1.415 1.392-1.919 1.294-.345-.067-1.087-.895-1.289-1.352-.145-.328-.405-1.154-.035-1.461zM8.11 11.537c.165-.783.783-1.18.801-1.196.313-.272.919-.518 1.55-.704.095-.025.154-.038.168-.041.39-.08.88-.213 1.204-.246.246-.025.549-.153.798.036.27.328.864.871 1.191.899.111.009.858-.224 2.083-1.321.153-.137.314-.272.484-.398l.049-.036c.376-.273.792-.5 1.244-.6.14-.031-.5.605.048 1.936.359.872 1.351 3.863 1.188 4.342-.097.286-.185.339-.432.225-.692-.321-1.544-1.269-3.181-1.908-1.319-.515-2.412-.503-3.12-.364-1.629.321-2.473 1.03-2.946 1.426-.072.061.701-.245 1.734-.362.936-.106 1.875-.004 3.037.786.742.621.561.573.226.666-1.28.356-3.719 1.092-4.57.968-.88-.129-1.966-2.388-1.556-4.108zm6.849 3.15c-1.028-1.225-2.201-1.604-2.948-1.654-.927-.062-1.594.057-2.226.267-.068.023 3.387-2.469 7.192.489.685.533 1.094.83 1.454 1.066.052.034-.917.29-1.081.336-1.575.261-2.008-.048-2.391-.504zm-.88.249c-.376.135-.903.321-1.477.499a61.891 61.891 0 0 1 1.477-.499zm-7.455 3.411c.06-.601.193-1.146.134-1.753-.001-.012 1.714 1.163 2.179 1.244.076.013-.801.812-1.697 1.188-.307-.04-.641-.428-.616-.679zm-1.882 4.935c-.092-.129-.694-2.51-.527-4.034.23-2.1 1.408-2.844 1.635-2.871.188-.022.56.1.386 2.198-.021.253 1.117 1.04 1.117 1.04-2.322 1.364-2.136 2.391-2.611 3.667zm4.754-2.535c-.117-.005-2.264-1.473-2.192-1.501 2.477-.964 2.656-2.467 2.827-2.849-.02-.032.484-.236 1.17-.491 1.104-.396 2.631-.891 3.158-1.086l.027-.008c.239-.069.249-.024.313.06.19.247.509.412.544.429.47.223 1.076.202 1.09.221.094.136.154 5.536-6.937 5.225zm7.31-2.681c-.021.001-.528-.472-.528-.472s.196-.427.295-.97c.08-.442.12-1.097.12-1.097s2.331-.278 2.435-1.082c.118-.918-.546-2.827-.645-3.139-.042-.132-.621-1.69-.706-2.093-.096-.454.087-1.187.36-1.294.643-.253 1.59.828 1.647 2.401.217 6.03-2.957 7.745-2.978 7.746z" /></ svg > } }