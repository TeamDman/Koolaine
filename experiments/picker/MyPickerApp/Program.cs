using System;
using System.Collections.Generic;
using System.Threading;
using System.Windows;
using System.Windows.Automation;
using System.Windows.Forms;

namespace MyCursorApp
{
    class Program
    {

        static void Main(string[] args)
        {
            while (true)
            {
                PrintUnderMouse();
                Thread.Sleep(1000);
            }
        }

        static void PrintUnderMouse()
        {
            Point cursorPosition = new Point(Cursor.Position.X, Cursor.Position.Y);
            Point point = cursorPosition;
            // Point point = new Point(-1447, 938);
            AutomationElement element = AutomationElement.FromPoint(point);

            if (element != null)
            {
                Console.WriteLine($"Element found at point {point}: {DetailsFor(element)}");

                Console.WriteLine("=== Gather Interesting ===");
                foreach (var item in GatherInteresting(element))
                {
                    (AutomationElement elem, int depth, string relation) = item;
                    string details = DetailsFor(elem);
                    Console.WriteLine($"{relation}: Depth: {depth} {details}");
                }
            }
            else
            {
                Console.WriteLine("No element found at the given coordinates.");
            }
        }

        static string DetailsFor(AutomationElement elem)
        {
            Rect boundingRect = (Rect)elem.GetCurrentPropertyValue(AutomationElement.BoundingRectangleProperty);
            string name = elem.Current.Name;
            string controlType = elem.Current.ControlType.ProgrammaticName;
            string className = elem.Current.ClassName;
            string automationId = elem.Current.AutomationId;
            string value = GetValue(elem);  // Extracted value fetching to a separate method

            return $"{name}, BoundingRect: {boundingRect}, ControlType: {controlType}, ClassName: {className}, AutomationId: {automationId}, Value: {value ?? "N/A"}";
        }

        static string GetValue(AutomationElement element)
        {
            object patternObj;
            if (element.TryGetCurrentPattern(ValuePattern.Pattern, out patternObj))
            {
                ValuePattern valuePattern = patternObj as ValuePattern;
                return valuePattern.Current.Value;
            }
            return null;  // or "N/A" or any other default value
        }

        static IEnumerable<(AutomationElement, int, string)> GatherInteresting(AutomationElement element, int maxDepth = 2)
        {
            // Yield children of the element
            foreach (var child in GetDescendants(element, 0, maxDepth))
            {
                yield return (child.Item1, child.Item2, "Child");
            }

            // Yield siblings of the element
            TreeWalker walker = TreeWalker.ControlViewWalker;
            AutomationElement sibling = walker.GetNextSibling(element);
            while (sibling != null)
            {
                yield return (sibling, 0, "Sibling");
                sibling = walker.GetNextSibling(sibling);
            }

            // Yield siblings of the element's parent
            AutomationElement parent = walker.GetParent(element);
            if (parent != null)
            {
                AutomationElement parentSibling = walker.GetNextSibling(parent);
                while (parentSibling != null)
                {
                    yield return (parentSibling, 0, "Parent's Sibling");
                    parentSibling = walker.GetNextSibling(parentSibling);
                }
            }
        }

        static IEnumerable<AutomationElement> GetAncestors(AutomationElement element)
        {
            TreeWalker walker = TreeWalker.ControlViewWalker;
            AutomationElement parentElement = walker.GetParent(element);

            while (parentElement != null)
            {
                yield return parentElement;
                parentElement = walker.GetParent(parentElement);
            }
        }

        static IEnumerable<(AutomationElement, int)> GetDescendants(AutomationElement element, int currentDepth, int maxDepth)
        {
            if (currentDepth > maxDepth)
            {
                yield break;
            }

            TreeWalker walker = TreeWalker.ControlViewWalker;
            AutomationElement childElement = null;
            try {
                childElement = walker.GetFirstChild(element);
            } catch (Exception e){
                Console.WriteLine($"Encountered error gathering descendants on {DetailsFor(element)}");
                Console.WriteLine(e);
            }
            
            while (childElement != null)
            {
                yield return (childElement, currentDepth);

                foreach (var grandChild in GetDescendants(childElement, currentDepth + 1, maxDepth))
                {
                    yield return grandChild;
                }

                try {
                    childElement = walker.GetNextSibling(childElement);
                } catch (Exception e){
                    Console.WriteLine($"Gathering siblings failed on {DetailsFor(childElement)}");
                    Console.WriteLine(e);
                }
            }
        }
    }
}
