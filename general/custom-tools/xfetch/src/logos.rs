use colored::*;

pub fn get_logo(os_name: &str) -> String {
    let os_lower = os_name.to_lowercase();
    
    if os_lower.contains("windows") {
        get_windows_logo()
    } else if os_lower.contains("ubuntu") {
        get_ubuntu_logo()
    } else if os_lower.contains("debian") {
        get_debian_logo()
    } else if os_lower.contains("fedora") {
        get_fedora_logo()
    } else if os_lower.contains("arch") {
        get_arch_logo()
    } else if os_lower.contains("centos") || os_lower.contains("rhel") {
        get_centos_logo()
    } else if os_lower.contains("opensuse") || os_lower.contains("suse") {
        get_opensuse_logo()
    } else if os_lower.contains("mint") {
        get_mint_logo()
    } else if os_lower.contains("manjaro") {
        get_manjaro_logo()
    } else if os_lower.contains("linux") {
        get_generic_linux_logo()
    } else {
        get_generic_logo()
    }
}

fn get_windows_logo() -> String {
    format!(
        "{}",
        r#"                                ..,
                    ....,,:;+ccllll
      ...,,+:;  cllllllllllllllllll
,cclllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll

llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
llllllllllllll  lllllllllllllllllll
`'ccllllllllll  lllllllllllllllllll
       `' \*::  :ccllllllllllllllll
                       ````''*::cll
                                 ``"#
            .bright_blue()
    )
}

fn get_ubuntu_logo() -> String {
    format!(
        "{}",
        r#"         _
     ---(_)
 _/  ---  \
(_) |   |
  \  --- _/
     ---(_)

         _
     ---(_)
 _/  ---  \
(_) |   |
  \  --- _/
     ---(_)"#
            .bright_red()
    )
}

fn get_debian_logo() -> String {
    format!(
        "{}",
        r#"  _____
 /  __ \
|  /    |
|  \___-
-_
  --_"#
            .bright_red()
    )
}

fn get_fedora_logo() -> String {
    format!(
        "{}",
        r#"        ,''''.
       |   ,,;|
       |   ;;;'|
   ,,,;|   ';';
  ;;'  |     ;|
 ;;    |  ,;' |
|'     | ;;;  |
|      |;;'   |
 \     |;'   /
  \    |;   /
   \   |   /
    \  |  /
     \ | /
      \|/"#
            .bright_blue()
    )
}

fn get_arch_logo() -> String {
    format!(
        "{}",
        r#"                   -`
                  .o+`
                 `ooo/
                `+oooo:
               `+oooooo:
               -+oooooo+:
             `/:-:++oooo+:
            `/++++/+++++++:
           `/++++++++++++++:
          `/+++ooooooooo+++/`
         ./ooosssso++osssssso+`
        .oossssso-````/ossssss+`
       -osssssso.      :ssssssso.
      :osssssss/        osssso+++.
     /ossssssss/        +ssssooo/-
   `/ossssso+/:-        -:/+osssso+-
  `+sso+:-`                 `.-/+oso:
 `++:.                           `-/+/
 .`                                 `/"#
            .bright_cyan()
    )
}

fn get_centos_logo() -> String {
    format!(
        "{}",
        r#"                 ..
               .PLTJ.
              <><><><>
     KKSSV' 4KKK LJ KKKL.'VSSKK
     KKV' 4KKKKK LJ KKKKAL 'VKK
     V' ' 'VKKKK LJ KKKKV' ' 'V
        .4MA.' 'VKK LJ KKV' '.4Mb.
      . KKKKKA.' 'V LJ V' '.4KKKKK .
     .4D KKKKKKKA.'' LJ ''.4KKKKKKK FA.
    <QDD ++++++++++++  ++++++++++++ GFD>
     'VD KKKKKKKK'.. LJ ..'KKKKKKKK FV
       ' VKKKKK'. .4 LJ K. .'KKKKKV '
         'VK'. .4KK LJ KKA. .'KV'
        A. . .4KKKK LJ KKKKA. . .4
        KKA. 'KKKKK LJ KKKKK' .4KK
        KKSSA. VKKK LJ KKKV .4SSKK
               <><><><>
                'MKKM'
                  ''"#
            .bright_yellow()
    )
}

fn get_opensuse_logo() -> String {
    format!(
        "{}",
        r#"           .;ldkO0000Okdl;.
       .;d00xl:^''''''^:ok00d;.
     .d00l'                'o00d.
   .d0Kd'  Okxol:;,.          :O0d.
  .OK0yc  ;0MMMMMMMMMMMMKc.    cOKO.
 ,0K0yx:  ;KMMMMMMMMMMMMMKc   ,xOK0,
.OK0yc    ;KMMMMMMMMMMMMMKc    cOK0.
,0K0yx:   ;KMMMMMMMMMMMMMKc   ,xOK0,
.OK0yc    ;KMMMMMMMMMMMMMKc    cOK0.
 ,0K0yx:  ;KMMMMMMMMMMMMMKc   ,xOK0,
  .OK0yc  ;0MMMMMMMMMMMMKc.    cOKO.
   .d0Kd'  Okxol:;,.          :O0d.
     .d00l'                'o00d.
       .;d00xl:^''''''^:ok00d;.
           .;ldkO0000Okdl;."#
            .bright_green()
    )
}

fn get_mint_logo() -> String {
    format!(
        "{}",
        r#" MMMMMMMMMMMMMMMMMMMMMMMMMmds+.
 MMm----::-://////////////oymNMd+`
 MMd      /++                -sNMd:
 MMNso/`  dMM    `.::-. .-::.` .hMN:
 ddddMMh  dMM   :hNMNMNhNMNMNh: `NMm
     NMm  dMM  .NMN/-+MMM+-/NMN` dMM
     NMm  dMM  -MMm  `MMM   dMM. dMM
     NMm  dMM  -MMm  `MMM   dMM. dMM
     NMm  dMM  .mmd  `mmm   yMM. dMM
     NMm  dMM`  ..`   ...   ydm. dMM
     hMM- +MMd/-------...-:sdds  dMM
     -NMm- :hNMNNNmdddddddddy/`  dMM
      -dMNs-``-::::-------.``    dMM
       `/dMNmy+/:-------------:/yMMM
          ./ydNMMMMMMMMMMMMMMMMMMMMM
             \./sdmmmmmmmmmmmmmmmm"#
            .bright_green()
    )
}

fn get_manjaro_logo() -> String {
    format!(
        "{}",
        r#"██████████████████  ████████
██████████████████  ████████
██████████████████  ████████
██████████████████  ████████
████████            ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████
████████  ████████  ████████"#
            .bright_green()
    )
}

fn get_generic_linux_logo() -> String {
    format!(
        "{}",
        r#"        #####
       #######
       ##O#O##
       #VVVVV#
     ##  VVV  ##
    #          ##
   #            ##
   #            ###
  QQ#           ##Q
 QQQQ#         #QQQQ
 QQQQQ#       #QQQQQ
QQQQQQ#######QQQQQQQ"#
            .bright_yellow()
    )
}

fn get_generic_logo() -> String {
    format!(
        "{}",
        r#"   _____
  /     \
 |  O   O |
 |    >   |
 |  \___/ |
  \_____/

  Unknown OS"#
            .white()
    )
}