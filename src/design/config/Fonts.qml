import QtQuick
import QtQuick.Controls

QtObject {
    property var micro5: FontLoader {
        id: micro5
        source: "qrc:/fonts/Micro5.ttf"
    }
    property var vt323: FontLoader {
        id: vt323
        source: "qrc:/fonts/VT323.ttf"
    }
    property var notoSansMono: FontLoader {
        id: notoSansMono
        source: "qrc:/fonts/NotoSansMono.ttf"
    }

     property font logoFont: micro5.font
     property font altFont: vt323.font
     property font mainFont: notoSansMono.font
}


