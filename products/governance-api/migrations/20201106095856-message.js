"use strict";

module.exports = {
  up: async (queryInterface, Sequelize) => {
    return queryInterface.createTable("Message", {
      id: {
        allowNull: false,
        autoIncrement: true,
        primaryKey: true,
        type: Sequelize.INTEGER,
      },
      address: {
        allowNull: false,
        type: Sequelize.STRING,
      },
      version: {
        allowNull: false,
        type: Sequelize.STRING,
      },
      timestamp: {
        allowNull: false,
        type: Sequelize.INTEGER,
      },
      type: {
        allowNull: false,
        type: Sequelize.STRING,
      },
      space: {
        allowNull: false,
        type: Sequelize.STRING,
      },
      token: {
        allowNull: false,
        type: Sequelize.STRING,
      },
      author_ipfs_hash: {
        allowNull: false,
        unique: true,
        type: Sequelize.STRING,
      },
      payload: {
        allowNull: false,
        type: Sequelize.JSON,
      },
      proposal_id: {
        allowNull: true,
        type: Sequelize.STRING,
      },
      sig: {
        allowNull: false,
        type: Sequelize.JSON,
      },
      createdAt: {
        allowNull: false,
        type: Sequelize.DATE,
      },
      updatedAt: {
        allowNull: false,
        type: Sequelize.DATE,
      },
    });
  },

  down: async (queryInterface, Sequelize) => {
    return queryInterface.dropTable("Message");
  },
};
