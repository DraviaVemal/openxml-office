// Copyright (c) DraviaVemal. Licensed under the MIT License. See License in the project root.

using draviavemal.openxml_office.global_2007;
using draviavemal.openxml_office.presentation_2007;

namespace openxmloffice.tests
{
    /// <summary>
    /// Power Point Test
    /// </summary>
    [TestClass]
    public class Presentation
    {
        private static readonly string resultPath = "../../test_results";
        /// <summary>
        /// Initialize power point Test
        /// </summary>
        /// <param name="context">
        /// </param>
        [ClassInitialize]
        public static void ClassInitialize(TestContext context)
        {
            if (!Directory.Exists(resultPath))
            {
                Directory.CreateDirectory(resultPath);
            }
            PrivacyProperties.ShareComponentRelatedDetails = false;
            PrivacyProperties.ShareIpGeoLocation = false;
            PrivacyProperties.ShareOsDetails = false;
            PrivacyProperties.SharePackageRelatedDetails = false;
            PrivacyProperties.ShareUsageCounterDetails = false;
        }
        /// <summary>
        /// Save the Test File After execution
        /// </summary>
        [ClassCleanup]
        public static void ClassCleanup()
        {
            // powerPoint.SaveAs(string.Format("{1}/test-{0}.xlsx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
        }

        /// <summary>
        /// 
        /// </summary>
        // [TestMethod]
        public void BlankFile()
        {
            PowerPoint powerPoint2 = new(new()
            {
                IsInMemory = false
            });
            powerPoint2.SaveAs(string.Format("{1}/Blank-{0}.pptx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
        }

        /// <summary>
        /// Test existing file
        /// </summary>
        [TestMethod]
        public void OpenExistingPowerPoint()
        {
            PowerPoint powerPoint1 = new("test_files/basic_test.pptx", new()
            {
                IsInMemory = false
            });
            powerPoint1.SaveAs(string.Format("{1}/EditStyle-{0}.pptx", DateTime.Now.ToString("yyyy-MM-dd-HH-mm-ss"), resultPath));
            Assert.IsTrue(true);
        }
    }
}
