#ifndef TOOLS_H
#define TOOLS_H

#include <string>
#include <iostream>
#include <exception>

#include <QCoreApplication>
#include <QString>
#include <QFile>
#include <QJsonDocument>
#include <QJsonObject>

namespace tools
{
class NoSuchProfileError : public std::exception {
public:
    const char* what() const noexcept override;
};

QString getFullPath
    (
    QString path
    );

QJsonObject getJsonObject
    (
    const QString& filename
    );

void writeJson
    (
    const QString& filename,
    QJsonObject jsonObject
    );

void deleteFromObject
    (
    const QString& name,
    QJsonObject object,
    bool confirmationDialog = true
    );

int copyFilesWithExtention
    (
    const QString& sourceDir,
    const QString& destDir,
    const QStringList& extensions
    );

void updateProgressBar
    (
    int progress,
    int total
    );

QString installProcess
    (
    const QString& script,
    const QStringList& envVars = QStringList()
    );

QString interpretPath
    (
    const QString& path,
    QMap<QString, QString> replacements = {}
    );

const QString list
    (
    const QJsonObject& object
    );

void createPath
    (
        const QString& path
    );
}

#endif // TOOLS_H
