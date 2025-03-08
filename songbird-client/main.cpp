#include "mainwindow.h"
#include "constants.h"
#include <QApplication>
#include <QStyleFactory>

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);

    // Apply a modern style
    a.setStyle(QStyleFactory::create("Fusion"));

    // Set application properties
    a.setApplicationName(Constants::PROJECT_NAME);
    a.setApplicationVersion(Constants::VERSION);

    // Create and show the main window
    MainWindow w;
    w.show();

    return a.exec();
}
