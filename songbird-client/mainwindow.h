#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QStackedWidget>
#include "loginscreen.h"
#include "registrationscreen.h"
#include "user.h"

QT_BEGIN_NAMESPACE
namespace Ui {
class MainWindow;
}
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    MainWindow(QWidget *parent = nullptr);
    ~MainWindow();

public slots:
    void showLoginScreen();
    void showRegistrationScreen();
    void showMainInterface(User user);

private:
    Ui::MainWindow *ui;
    QStackedWidget *m_stackedWidget;
    LoginScreen *m_loginScreen;
    RegistrationScreen *m_registrationScreen;
    QWidget *m_mainInterface;

    void setupMainInterface();
};
#endif // MAINWINDOW_H
