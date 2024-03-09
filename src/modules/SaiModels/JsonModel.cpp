#include "JsonModel.hpp"

JsonModel::JsonModel(QObject * parent) : QAbstractListModel(parent) { }
JsonModel::~JsonModel() {}

int JsonModel::rowCount(const QModelIndex & parent) const
{
    if (parent.isValid() || this->object_.isEmpty())
        return 0;
    return this->object_.count();
}

QVariant JsonModel::data(const QModelIndex &index, int role) const
{
    if (!index.isValid() || index.row() >= this->object_.size())
        return QVariant();

    QString key = this->object_.keys().at(index.row());
    QString value = this->object_.value(key).toString();

    switch (role) {
    case KeyRole:
        return key;
    case ValueRole:
        return value;
    default:
        return QVariant();
    }
}

QHash<int, QByteArray> JsonModel::roleNames() const
{
    QHash<int, QByteArray> roles;
    roles[KeyRole] = "key";
    roles[ValueRole] = "value";
    return roles;
}

void JsonModel::init(const QJsonObject & object)
{
    this->beginResetModel();
    this->object_ = object;
    this->endResetModel();
}
