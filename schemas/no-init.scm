{
    "$schema": "http://json-schema.org/draft-07/schema#",
    "type": "object",
    "properties": {
        "title": {
            "type": "string"
        },
        "desc": {
            "type": "string"
        },
        "inputs": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "object": {
                        "type": "object",
                        "properties": {
                            "type": {
                                "type": "string"
                            },
                            "display": {
                                "type": "string"
                            },
                            "dtype": {
                                "type": "string"
                            },
                            "dims": {
                                "type": "array",
                                "items": {
                                    "type": [
                                        "integer",
                                        "string"
                                    ]
                                }
                            }
                        },
                        "required": [
                            "type",
                            "display",
                            "dtype",
                            "dims"
                        ]
                    },
                    "desc": {
                        "type": "string"
                    }
                },
                "required": [
                    "object",
                    "desc"
                ]
            }
        },
        "outputs": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "object": {
                        "type": "object",
                        "properties": {
                            "type": {
                                "type": "string"
                            },
                            "display": {
                                "type": "string"
                            },
                            "dtype": {
                                "type": "string"
                            },
                            "dims": {
                                "type": "array",
                                "items": {
                                    "type": [
                                        "integer",
                                        "string"
                                    ]
                                }
                            }
                        },
                        "required": [
                            "type",
                            "display",
                            "dtype",
                            "dims"
                        ]
                    },
                    "desc": {
                        "type": "string"
                    }
                },
                "required": [
                    "object",
                    "desc"
                ]
            }
        },
        "construct": {
            "type": "string"
        },
        "args": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "id": {
                        "type": "string"
                    },
                    "object": {
                        "type": "object",
                        "properties": {
                            "type": {
                                "type": "string"
                            },
                            "display": {
                                "type": "string"
                            }
                        },
                        "required": [
                            "type",
                            "display"
                        ]
                    },
                    "desc": {
                        "type": "string"
                    }
                },
                "required": [
                    "id",
                    "object",
                    "desc"
                ]
            }
        },
        "dependencies": {
            "type": "array",
            "items": {
                "type": "string"
            }
        },
        "device-agnostic": {
            "type": "boolean"
        }
    },
    "required": [
        "title",
        "desc",
        "inputs",
        "outputs",
        "construct",
        "args",
        "dependencies",
        "device-agnostic"
    ]
}