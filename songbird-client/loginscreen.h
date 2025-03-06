#ifndef LOGINSCREEN_H
#define LOGINSCREEN_H

#include <QDialog>
#include <QLabel>
#include <QLineEdit>
#include <QPushButton>
#include <QCheckBox>
#include <QGridLayout>
#include <QMessageBox>



class LoginScreen : public QDialog
{
public:
    LoginScreen(QWidget *parent = nullptr);
private slots:
    void onLoginClicked();
    void onForgotPasswordClicked();
    void onRegisterClicked();
private:
    void saveCredentials(const QString &username);
    QWidget *parent;
    QLineEdit *m_usernameEdit;
    QLineEdit *m_passwordEdit;
    QCheckBox *m_rememberCheckBox;
    QPushButton *m_loginButton;
    QPushButton *m_cancelButton;
    QPushButton *m_registerButton;
    QLabel *m_forgotPasswordLabel;
};


#endif // LOGINSCREEN_H
