#include "mainwindow.h"
#include "constants.h"
#include "loginscreen.h"
#include <QApplication>

int main(int argc, char *argv[])
{
    QApplication a(argc, argv);
    MainWindow w;
    w.setWindowTitle(Constants::PROJECT_NAME);
    LoginScreen log(&w);
    log.show();
    return a.exec();
}
