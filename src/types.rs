#[derive(Clone, Debug)]
pub struct TableMaster {
    // zip压缩包文件名
    pub raw_filename: String,
    // 包表时间
    pub baobiaotime: String,
    // 时间戳
    pub timestamp: u64,

    // unzip -l 查看到的csv文件名，
    // 有可能是相对路径
    // Archive:  2023-10-14 06:31:43_ODM-A.WL.PM.FILE_WL_4G_HW_IMS_SBC_V4.3.0_PM_842_tpd_dns_q_20231014060000-20231014061500_X1697236205631139.zip
    //   Length      Date    Time    Name
    // ---------  ---------- -----   ----
    //      9049  10-14-2023 06:31   ODM-A.WL.PM.FILE_WL_4G_HW_IMS_SBC_V4.3.0_PM_842_tpd_dns_q_20231014060000_20231014061500_X1697236205631139/ODM-A.WL.PM.FILE_WL_4G_HW_IMS_SBC_V4.3.0_PM_842_tpd_dns_q_20231014060000-20231014061500_X1697236205631139.xml
    //       702  10-14-2023 06:31   ODM-A.WL.PM.FILE_WL_4G_HW_IMS_SBC_V4.3.0_PM_842_tpd_dns_q_20231014060000_20231014061500_X1697236205631139/ODM-A.WL.PM.FILE_WL_4G_HW_IMS_SBC_V4.3.0_PM_842_tpd_dns_q_20231014060000-20231014061500_X1697236205631139.csv
    // ---------                     -------
    //      9751                     2 files
    // 也有可能是文件名
    // Archive:  ODM-A.WL.PM.FILE_WL_IMS_ZTE_IPSMGW_tpd_ip_smgw_q_20231014023000-20231014023015_XG1019T20231014033000AA8202merge.zip
    //   Length      Date    Time    Name
    // ---------  ---------- -----   ----
    //      2185  10-14-2023 03:30   ODM-A.WL.PM.FILE_WL_IMS_ZTE_IPSMGW_tpd_ip_smgw_q_20231014023000-20231014023015_XG1019T20231014033000AA8202merge.csv
    //     23201  10-14-2023 03:30   ODM-A.WL.PM.FILE_WL_IMS_ZTE_IPSMGW_tpd_ip_smgw_q_20231014023000-20231014023015_XG1019T20231014033000AA8202merge.xml
    // ---------                     -------
    //     25386                     2 files
    pub csv_name: String,
    // csv文件内容行数
    pub csv_count: u64,
}

pub trait Update {
    fn update( &self, other: TableMaster) -> TableMaster ;

}

//impl Update for TableMaster {
//    fn update( &self, other: TableMaster ) -> TableMaster {
//
//       // 因为只要最新，所以不判断新包旧包csv_count大小
//        if self.baobiaotime == other.baobiaotime && self.timestamp  < other.timestamp { 
//            TableMaster { ..other }
//        } else {
//            TableMaster { &self.raw_filename, &self.baobiaotime, &self.timestamp, &self.csv_count, &self.csv_name, }
//        }
//
//    }
//
//}
