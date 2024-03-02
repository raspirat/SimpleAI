#ifndef CLAI_H
#define CLAI_H

#include <QObject>
#include <QJsonObject>
#include "external/simpleClai/src/config/config.h"
#include "external/simpleClai/src/utils/tools.h"
#include "external/simpleClai/src/commands/project.h"
#include "external/simpleClai/src/commands/profile.h"
#include "external/simpleClai/src/commands/model.h"
#include "external/simpleClai/src/commands/dataset.h"

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
    void createProject(QString&, QString&, QString&);
    void deleteProject(QString&);
};

#endif // CLAI_H
