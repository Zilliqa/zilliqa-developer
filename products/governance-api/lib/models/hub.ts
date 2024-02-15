import {
  Table,
  Column,
  Model,
  CreatedAt,
  UpdatedAt,
} from "sequelize-typescript";

@Table
export class Hub extends Model<Hub> {
  @Column
  host!: string;

  @Column
  address!: string;

  @Column
  is_self: boolean = false;

  @Column
  is_active: boolean = true;

  @CreatedAt
  @Column
  createdAt!: Date;

  @UpdatedAt
  @Column
  updatedAt!: Date;
}
