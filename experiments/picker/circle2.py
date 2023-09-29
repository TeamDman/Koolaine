import sys
import pyautogui
from PyQt5.QtCore import Qt, QTimer, QPoint
from PyQt5.QtGui import QPainter, QBrush, QColor, QFont
from PyQt5.QtWidgets import QApplication, QMainWindow, QWidget

# Global variables for circle parameters
circle_radius = 50
outline_width = 2
outline_color = Qt.white

class GameOverlay(QWidget):
    def __init__(self):
        super().__init__()

        self.setWindowFlags(Qt.FramelessWindowHint | Qt.WindowStaysOnTopHint)
        self.setAttribute(Qt.WA_TranslucentBackground)

        # Allow mouse events to pass through
        self.setAttribute(Qt.WA_TransparentForMouseEvents, True)

        screen = QApplication.primaryScreen()
        screen_geometry = screen.geometry()

        screen_width = screen_geometry.width()
        screen_height = screen_geometry.height()

        self.setGeometry(0, 0, screen_width, screen_height)
        self.show()

    def paintEvent(self, event):
        painter = QPainter(self)
        painter.setRenderHint(QPainter.Antialiasing)

        # Draw a transparent background
        painter.setBrush(QBrush(QColor(0, 0, 0, 0)))
        painter.drawRect(self.rect())

        # Get the cursor position using pyautogui
        cursor_pos = pyautogui.position()
        circle_center_x = cursor_pos.x
        circle_center_y = cursor_pos.y

        # Draw the circle
        painter.setPen(QColor(outline_color))
        painter.setBrush(QBrush(QColor(0, 0, 0, 0)))
        painter.drawEllipse(circle_center_x - circle_radius, circle_center_y - circle_radius,
                            circle_radius * 2, circle_radius * 2)

        # Draw the text
        painter.setFont(QFont("Arial", 16))
        painter.drawText(QPoint(circle_center_x + circle_radius + 10, circle_center_y), "Hello world")

def main():
    app = QApplication(sys.argv)
    overlay = GameOverlay()

    timer = QTimer()
    timer.timeout.connect(overlay.update)
    timer.start(1)

    sys.exit(app.exec_())

if __name__ == '__main__':
    main()
