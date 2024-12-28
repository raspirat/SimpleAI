#ifndef JSONMODEL_H
#define JSONMODEL_H

#include <QtQml/qqmlregistration.h>
#include <QObject>
#include <QJsonObject>
#include <QAbstractListModel>
#include <QJsonArray>

class JsonModel : public QAbstractListModel
{

Q_OBJECT
QML_ELEMENT

private:
    enum Roles {
        KeyRole = Qt::UserRole + 1,
        ValueRole
    };
    QJsonObject object_ { };
public:
    explicit JsonModel(QObject * parent = nullptr);
    ~JsonModel();
    int rowCount(const QModelIndex & parent = QModelIndex()) const override;
    QVariant data(const QModelIndex & index, int role = Qt::DisplayRole) const override;
    QHash<int, QByteArray> roleNames() const override;
public slots:
    void init(const QJsonObject & object);
};


#endif
