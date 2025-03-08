#ifndef REGISTRATIONSCREEN_H
#define REGISTRATIONSCREEN_H

#include <QWidget>
#include <QLabel>
#include <QLineEdit>
#include <QPushButton>
#include <QGridLayout>
#include <QMessageBox>
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QFrame>
#include "user.h"

class RegistrationScreen : public QWidget
{
    Q_OBJECT

public:
    explicit RegistrationScreen(QWidget *parent = nullptr);

signals:
    void registrationCompleted(User user);
    void registrationCancelled();

private slots:
    void onRegisterClicked();
    void onCancelClicked();

private:
    bool validateForm();
    void submitNewUser(User &user);

    QLineEdit *m_usernameEdit;
    QLineEdit *m_emailEdit;
    QLineEdit *m_passwordEdit;
    QLineEdit *m_passwordConfirmEdit;
    QPushButton *m_registerButton;
    QPushButton *m_cancelButton;
};

#endif // REGISTRATIONSCREEN_H
