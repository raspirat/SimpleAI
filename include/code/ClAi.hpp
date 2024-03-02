#ifndef CLAI_H
#define CLAI_H

#include <QObject>
#include <QJsonObject>
#include "external/simpleClai/src/config/config.h"
#include "external/simpleClai/src/utils/tools.h"

class ClAi : public QObject
{
    Q_OBJECT
public:
    ClAi();
public slots:
    static const QString getDatasetsConfigPath();
    static const QString getProfilesConfigPath();
    static const QString getProjectsConfigPath();
    static QJsonObject getDatasetsJson();
    static QJsonObject getProfilesJson();
    static QJsonObject getProjectsJson();
    static QJsonObject getModelsJson();
    static bool saveJson(const QJsonObject & object, const QString & filePath);
};

#endif // CLAI_H
